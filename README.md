# Overview

Example of listing kubernetes pods on the current node using in-cluster
credentials. Uses [k8s-openapi] bindings and [isahc] HTTP client to have
lighter synchronous dependencies than [kube-rs].

# Setup

Developed using [KinD] (Kubernetes in Docker) but can be used with any
kubernetes cluster provided the image `rossf7/rust-k8s-test:dev` is present.

```sh
kind create cluster
```

# Run example

Build the docker image and run it via a kubernetes job.

```sh
docker build -t rossf7/rust-k8s-test:dev .
kind load docker-image rossf7/rust-k8s-test:dev
kubectl delete -f job.yaml
kubectl apply -f job.yaml
kubectl wait --for=condition=complete job/rust-k8s-test
kubectl logs -l app=rust-k8s-test
```

Logs output should look like below.

```sh
K8sContainer { container_id: "9bb9590067cfed4210e971affd81c7aa367251fc7cc20fc8c165e17459ddfc64", container_name: "rust-k8s-test", namespace: "default", labels: {"app": "rust-k8s-test", "controller-uid": "b583dbc2-6ca5-4bd1-aee9-560d2dd3124e", "job-name": "rust-k8s-test"}, pod_name: "rust-k8s-test-lc7ck" }
K8sContainer { container_id: "c43f71055a0d0a647849f7377183a44771d8f4d56796d273d417c3399fa269c8", container_name: "coredns", namespace: "kube-system", labels: {"k8s-app": "kube-dns", "pod-template-hash": "558bd4d5db"}, pod_name: "coredns-558bd4d5db-l2rqb" }
...
```

[isahc]: https://github.com/sagebind/isahc
[k8s-openapi]: https://github.com/Arnavion/k8s-openapi
[kube-rs]: https://github.com/clux/kube-rs
[KinD]: https://kind.sigs.k8s.io/
