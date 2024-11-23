user := "root"
server := env("HETZNER_IP")
key := "~/.ssh/hetzner"
path := "/root/pizza-htmx"

watch:
    cargo watch -w src/ -w templates/ -x run

deploy:
    ssh -i ~/.ssh/hetzner root@{{ server }} "mkdir -p {{ path }}"
    scp -i ~/.ssh/hetzner -r . root@{{ server }}:{{ path }}

build_remote:
    ssh -i ~/.ssh/hetzner root@{{ server }} "cd {{ path }} && cargo build --release"
