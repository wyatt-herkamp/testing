#!/usr/bin/env bash

set -eou pipefail

file=${1}

cat <<-'EOT' >> $file
Please check the benchmark results for any regressions.
<details>
<summary>Benchmark results</summary>


```
EOT

cargo bench -q | tee -a $file

cat <<-'EOT' >> $file
```
</details>
EOT
