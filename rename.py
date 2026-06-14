import os

files_to_edit = [
    '.github/ISSUE_TEMPLATE/bug_report.md',
    '.github/workflows/release.yml',
    'CONTRIBUTING.md',
    'Cargo.toml',
    'README.md',
    'install.sh',
    'src/main.rs',
    'src/ui.rs',
    'scratch.rs',
]

replacements = [
    ('CodeLens', 'Livediff'),
    ('codelens', 'Livediff'),
    ('livediff', 'Livediff'),
    ('LiveDiff', 'Livediff'),
    ('CODELENS', 'LIVEDIFF'),
]

for f in files_to_edit:
    if os.path.exists(f):
        with open(f, 'r') as file:
            content = file.read()
        for old, new in replacements:
            content = content.replace(old, new)
        with open(f, 'w') as file:
            file.write(content)
print('Renamed all successfully')
