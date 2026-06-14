#!/usr/bin/env python3
"""Publish or prepare Livediff marketing assets through public APIs.

Safety defaults:
- DEV.to is created as an unpublished draft unless --devto-publish is passed.
- Reddit is dry-run unless --reddit-submit is passed with an explicit subreddit.
- No voting, mass posting, or duplicate community posting is implemented.
"""
from __future__ import annotations

import argparse
import base64
import json
import os
import re
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
DEVTO_ARTICLE = ROOT / "docs/marketing/devto-article.md"
REDDIT_PLAYBOOK = ROOT / "docs/marketing/reddit-playbook.md"
STATUS_FILE = ROOT / "docs/marketing/api-run-status.json"
GITHUB_REPO = "SoCkEt7/Livediff"
GITHUB_DESCRIPTION = "Rust TUI for live file diffs in the terminal."
GITHUB_HOMEPAGE = "https://socket7.github.io/Livediff/"
GITHUB_TOPICS = [
    "cli",
    "command-line",
    "developer-tools",
    "diff",
    "file-watcher",
    "monitoring",
    "productivity",
    "rust",
    "rust-cli",
    "terminal",
    "terminal-ui",
    "tui",
]


def env(name: str) -> str | None:
    value = os.getenv(name)
    return value if value else None


def request_json(
    method: str,
    url: str,
    *,
    headers: dict[str, str] | None = None,
    body: dict[str, Any] | None = None,
    form: dict[str, str] | None = None,
    basic_auth: tuple[str, str] | None = None,
) -> Any:
    data: bytes | None = None
    req_headers = {"User-Agent": "livediff-marketing-api/1.0"}
    if headers:
        req_headers.update(headers)
    if body is not None:
        data = json.dumps(body).encode("utf-8")
        req_headers["Content-Type"] = "application/json"
    if form is not None:
        data = urllib.parse.urlencode(form).encode("utf-8")
        req_headers["Content-Type"] = "application/x-www-form-urlencoded"
    if basic_auth:
        token = base64.b64encode(f"{basic_auth[0]}:{basic_auth[1]}".encode()).decode()
        req_headers["Authorization"] = f"Basic {token}"

    req = urllib.request.Request(url, data=data, headers=req_headers, method=method)
    try:
        with urllib.request.urlopen(req, timeout=30) as res:
            raw = res.read().decode("utf-8")
            return json.loads(raw) if raw else {"status": res.status}
    except urllib.error.HTTPError as exc:
        detail = exc.read().decode("utf-8", errors="replace")
        raise RuntimeError(f"{method} {url} failed: HTTP {exc.code}: {detail}") from exc


def split_frontmatter(path: Path) -> tuple[dict[str, Any], str]:
    text = path.read_text(encoding="utf-8")
    if not text.startswith("---\n"):
        return {}, text
    _, fm, body = text.split("---", 2)
    meta: dict[str, Any] = {}
    for line in fm.splitlines():
        if ":" not in line:
            continue
        key, value = line.split(":", 1)
        value = value.strip().strip('"')
        if value.lower() == "true":
            meta[key.strip()] = True
        elif value.lower() == "false":
            meta[key.strip()] = False
        elif key.strip() == "tags":
            meta[key.strip()] = [tag.strip() for tag in value.split(",") if tag.strip()]
        else:
            meta[key.strip()] = value
    return meta, body.strip() + "\n"


def update_github() -> dict[str, Any]:
    token = env("GH_TOKEN") or env("GITHUB_TOKEN")
    if not token:
        return {"status": "skipped", "reason": "GH_TOKEN/GITHUB_TOKEN missing"}
    headers = {
        "Authorization": f"Bearer {token}",
        "Accept": "application/vnd.github+json",
        "X-GitHub-Api-Version": "2022-11-28",
    }
    repo_url = f"https://api.github.com/repos/{GITHUB_REPO}"
    request_json("PATCH", repo_url, headers=headers, body={"description": GITHUB_DESCRIPTION, "homepage": GITHUB_HOMEPAGE})
    request_json("PUT", f"{repo_url}/topics", headers=headers, body={"names": GITHUB_TOPICS})
    current = request_json("GET", repo_url, headers=headers)
    return {
        "status": "updated",
        "url": current.get("html_url"),
        "description": current.get("description"),
        "homepage": current.get("homepage"),
    }


def create_devto_article(*, publish: bool) -> dict[str, Any]:
    key = env("DEVTO_API_KEY") or env("DEV_TO_API_KEY")
    if not key:
        return {"status": "skipped", "reason": "DEVTO_API_KEY/DEV_TO_API_KEY missing"}
    meta, body = split_frontmatter(DEVTO_ARTICLE)
    article = {
        "title": meta.get("title", "I built a live diff monitor for the terminal in Rust"),
        "published": bool(publish),
        "body_markdown": body,
        "tags": meta.get("tags", ["rust", "opensource", "productivity", "cli"]),
        "canonical_url": meta.get("canonical_url", f"https://github.com/{GITHUB_REPO}"),
        "description": meta.get("description", "A Rust TUI that watches file changes and shows real-time diffs."),
    }
    created = request_json(
        "POST",
        "https://dev.to/api/articles",
        headers={"api-key": key, "Accept": "application/vnd.forem.api-v1+json"},
        body={"article": article},
    )
    return {
        "status": "published" if publish else "draft_created",
        "id": created.get("id"),
        "url": created.get("url"),
        "published": created.get("published"),
    }


def reddit_token() -> str | None:
    required = ["REDDIT_CLIENT_ID", "REDDIT_CLIENT_SECRET", "REDDIT_USERNAME", "REDDIT_PASSWORD"]
    if any(not env(k) for k in required):
        return None
    token = request_json(
        "POST",
        "https://www.reddit.com/api/v1/access_token",
        form={
            "grant_type": "password",
            "username": env("REDDIT_USERNAME") or "",
            "password": env("REDDIT_PASSWORD") or "",
            "scope": "submit identity",
        },
        basic_auth=(env("REDDIT_CLIENT_ID") or "", env("REDDIT_CLIENT_SECRET") or ""),
        headers={"User-Agent": env("REDDIT_USER_AGENT") or "livediff-marketing-api/1.0 by script"},
    )
    return token.get("access_token")


def extract_reddit_draft(name: str) -> tuple[str, str]:
    text = REDDIT_PLAYBOOK.read_text(encoding="utf-8")
    pattern = rf"## {re.escape(name)}.*?\n\nTitle: `([^`]+)`\n\nBody:\n\n(.*?)(?=\n## Draft|\n## Comment|\Z)"
    match = re.search(pattern, text, re.S)
    if not match:
        raise RuntimeError(f"Reddit draft not found: {name}")
    body = match.group(2).strip()
    body = body.replace("\n> ", "\n").removeprefix("> ").replace("\n>", "\n")
    return match.group(1), body


def submit_reddit(*, subreddit: str | None, draft: str, submit: bool) -> dict[str, Any]:
    title, body = extract_reddit_draft(draft)
    if not submit:
        return {"status": "dry_run", "subreddit": subreddit, "title": title, "body_chars": len(body)}
    if not subreddit:
        return {"status": "skipped", "reason": "--subreddit required for real Reddit submit"}
    token = reddit_token()
    if not token:
        return {"status": "skipped", "reason": "Reddit API credentials missing"}
    result = request_json(
        "POST",
        "https://oauth.reddit.com/api/submit",
        headers={
            "Authorization": f"Bearer {token}",
            "User-Agent": env("REDDIT_USER_AGENT") or "livediff-marketing-api/1.0 by script",
        },
        form={
            "sr": subreddit,
            "kind": "self",
            "title": title,
            "text": body,
            "sendreplies": "true",
            "api_type": "json",
        },
    )
    return {"status": "submitted", "subreddit": subreddit, "response": result}


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--github", action="store_true", help="Update GitHub repo metadata through API")
    parser.add_argument("--devto", action="store_true", help="Create DEV.to article draft through API")
    parser.add_argument("--devto-publish", action="store_true", help="Publish DEV.to article instead of draft")
    parser.add_argument("--reddit", action="store_true", help="Prepare or submit Reddit post")
    parser.add_argument("--reddit-submit", action="store_true", help="Actually submit to Reddit")
    parser.add_argument("--subreddit", help="Subreddit name for Reddit submission")
    parser.add_argument("--reddit-draft", default="Draft 1", help="Draft name in reddit-playbook.md")
    parser.add_argument("--all", action="store_true", help="Run GitHub, DEV.to, and Reddit paths")
    args = parser.parse_args()

    if not any([args.github, args.devto, args.reddit, args.all]):
        args.all = True

    results: dict[str, Any] = {"timestamp": int(time.time())}
    if args.github or args.all:
        results["github"] = update_github()
    if args.devto or args.all:
        results["devto"] = create_devto_article(publish=args.devto_publish)
    if args.reddit or args.all:
        results["reddit"] = submit_reddit(subreddit=args.subreddit, draft=args.reddit_draft, submit=args.reddit_submit)

    STATUS_FILE.write_text(json.dumps(results, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(json.dumps(results, indent=2, ensure_ascii=False))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
