user := "root"
server := env("HETZNER_IP")
key := "~/.ssh/hetzner"
path := "/root/pizza-htmx"

watch:
    cargo watch -w src/ -w templates/ -x run

deploy:
    ssh -i ~/.ssh/hetzner root@{{ env("HETZNER_IP") }} "mkdir -p {{ path }}"
    scp -i ~/.ssh/hetzner -r . root@{{ env("HETZNER_IP") }}:{{ path }}

run_remote:
    ssh -i ~/.ssh/hetzner "cd {{ path }} && cargo run --release"
