# there
Run programs not here, but `there(1)`.

## Idea
Managing `kubernetes` clusters can be a hassle. Between `kubectl(1)`,
`gcloud(1)`, loading configuration files, `terraform(1)`, `docker(1)`,
`helm(1)` it can be quite a chore to get a cluster running.
\
`there(1)` offers a coherent interface to interact with computers that are not
here, but `there(1)` on the `kubernetes` cluster. Hurray for cohesive
interfaces.

## usage
```txt
there(1) - Manage remote computers

Commands
  config   Configure your local tool
  init     Create a new repository locally
  build    Build a new container and upload it to your registry
  status   Display the status of your remote computers
  deploy   Deploy a container from your registry to your computers

Flags
  -h, --help      Print this message
  -v, --version   Print version information

Examples
  $ there init
  $ there build service-gateway
  $ there status
  $ there deploy
```

## FAQ
### Why are you building this in Rust?
`rust` is fun, and I've got to learn it at some point soooo...

### Why are there shell scripts all over the place?
Well, I had a bunch of `shell` scripts already, so figured I might as well go
ahead and wrap them in `rust` until something more usable rolls out.

### Is this production ready?
No, and it might never be. Though I'll totally be using it for my own
infrastructure the moment I can get it to run. Swoosh.

## See Also
- https://github.com/yoshuawuyts/infra
- https://github.com/yoshuawuyts/knowledge/tree/master/bin/kubectl.md
- https://github.com/yoshuawuyts/knowledge/blob/master/bin/gcloud.md
- https://github.com/yoshuawuyts/knowledge/blob/master/bin/docker.md
- [sup](https://github.com/pressly/sup)

## License
[MIT](https://tldrlegal.com/license/mit-license)
