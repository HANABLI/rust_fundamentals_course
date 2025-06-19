#!/usr/bin/env bash
## Clean all directories in the repository using cargo clean
echo "Start cleaning."

for DIR in */; do
    DIRNAME=$(basename "$DIR")
    echo "==> $DIRNAME <=="
    (cd "$DIR" && cargo clean --quiet )
done

echo "Clean complete."