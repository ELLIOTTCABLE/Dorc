#!/bin/sh
# The loop body is one aggregate replacement over two member facts.
for pkg in nginx curl; do
   apt-get install -y "$pkg"
done
