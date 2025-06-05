resource "kubernetes_namespace" "nginx" {
  metadata {
    name = var.nginx_ns
  }
}

resource "helm_release" "nginx" {
  name       = "ingress-nginx"
  repository = "https://kubernetes.github.io/ingress-nginx"
  chart      = "ingress-nginx"
  version    = "4.0.19"
  namespace  = var.nginx_ns

  values = [templatefile("${path.module}/templates/nginx.values.tftpl", {
    ingress_class = var.ingress_class
    dns_prefix    = var.dns_prefix
  })]

  depends_on = [
    kubernetes_namespace.nginx
  ]
}
