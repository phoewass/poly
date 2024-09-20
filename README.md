# Poly

### Poly
Poly means many or much from Ancient Greek polús, it's a play on words so the opposite of Mono-repo becomes Poly-repo.
Poly is heavily inspired from mani and garden
also some inspi

### Motivation
Working on different multi-repo projects that usually use multiple (micro)services, 
there are few steps needed, like a glue, to run the full project locally.
Poly aims to act like glue that enables developers to run multi-repo projects locally.

### Install

### Developer

### Configuration

```yaml
# poly.yaml
projects:
  frontend:
    # repo: 
    #   url: https://example.com
    path: ../project-one/app
    tags: [frontend, node, all]
  api:
    # repo: 
    #   url: https://example.com
    path: ../project-one/api
    tags: [api, node, all]
  project-two:
    # repo: 
    #   url: https://example.com
    path: ../project-two
    tags: [backend, all]

tasks:
  task-one:
    variables:
      HELLO: world
    commands:
      - name: display variables
        cmd: echo $HELLO
        run_on: api

```