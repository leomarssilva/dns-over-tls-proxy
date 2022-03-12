#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage:"
    echo "./random-list.sh random-ip-list"
    exit 1
fi;

domains=("alfa" "beta" "gama" "delta")

index=0
while read -r ip; do
    domain="${domains[((index % 4))]}"
    random_name="$(openssl rand -hex 12)"
    printf "%-45.40s%s_%03d.testinternal.%s\n" "$ip" "$random_name" "$index" "$domain"
    ((index++))
done < "$1" | sort -R