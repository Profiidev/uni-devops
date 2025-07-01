resource "kubernetes_namespace" "nats_ns" {
  metadata {
    name = "nats"
  }
}

resource "helm_release" "nats" {
  name       = "nats"
  repository = "https://nats-io.github.io/k8s/helm/charts"
  chart      = "nats"
  version    = "1.3.3"
  namespace  = kubernetes_namespace.nats_ns.metadata[0].name

  values = [templatefile("${path.module}/templates/nats.values.tftpl", {
  })]

  depends_on = [kubernetes_namespace.nats_ns]
}
