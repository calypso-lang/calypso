#!/usr/bin/env bash
echo "Installing pre-commit hook..."
cp .etc/pre-commit .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
echo "Installing commit-msg hook..."
cp .etc/commit-msg .git/hooks/commit-msg
chmod +x .git/hooks/commit-msg
echo "Done!"
