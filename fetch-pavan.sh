#!/bin/bash

git fetch pavan
for inp in $(git ls-tree --name-only pavan/main input/.); do
  filename=$(basename $inp)
  git show pavan/main:$inp > pavan/$filename
done
