resource "kubernetes_namespace" "nginx" {
  metadata {
    name = "nginx"
  }
}

resource "helm_release" "nginx" {
  name       = "ingress-nginx"
  repository = "https://kubernetes.github.io/ingress-nginx"
  chart      = "ingress-nginx"
  version    = "4.0.19"
  namespace  = "nginx"

  values = [templatefile("${path.module}/templates/nginx.values.tftpl", {
    ingress_class    = var.ingress_class
    resource_group   = var.k8s_resource_group_name
    pip_name         = var.public_ip_name
    load_balancer_ip = var.public_ip
  })]

  depends_on = [
    kubernetes_namespace.nginx
  ]
}
