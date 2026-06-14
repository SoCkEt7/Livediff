# API automation

Script: `scripts/marketing_api.py`

## Environment variables

### GitHub

```bash
export GH_TOKEN=...
# or
export GITHUB_TOKEN=...
```

### DEV.to

```bash
export DEVTO_API_KEY=...
# or
export DEV_TO_API_KEY=...
```

Default behavior creates an unpublished draft. Use `--devto-publish` only when the article is reviewed.

### Reddit

```bash
export REDDIT_CLIENT_ID=...
export REDDIT_CLIENT_SECRET=...
export REDDIT_USERNAME=...
export REDDIT_PASSWORD=...
export REDDIT_USER_AGENT='linux:livediff-marketing:1.0 by u/YOUR_USERNAME'
```

Default behavior is dry-run. Real posting requires both `--reddit-submit` and `--subreddit <name>`.

## Commands

Update GitHub metadata:

```bash
python3 scripts/marketing_api.py --github
```

Create DEV.to draft:

```bash
python3 scripts/marketing_api.py --devto
```

Publish DEV.to directly after review:

```bash
python3 scripts/marketing_api.py --devto --devto-publish
```

Dry-run Reddit draft:

```bash
python3 scripts/marketing_api.py --reddit --reddit-draft 'Draft 1'
```

Submit to Reddit after reading subreddit rules:

```bash
python3 scripts/marketing_api.py --reddit --reddit-submit --subreddit rust --reddit-draft 'Draft 1'
```

Run everything safely:

```bash
python3 scripts/marketing_api.py --all
```

## Safety

- No duplicate mass posting.
- No vote requests.
- No automatic Reddit posting without explicit subreddit.
- DEV.to stays draft unless `--devto-publish` is passed.
