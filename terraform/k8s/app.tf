resource "kubernetes_namespace" "app" {
  metadata {
    name = "app"
  }
}

resource "kubernetes_secret" "image_pull" {
  metadata {
    name      = "image-pull-secret"
    namespace = kubernetes_namespace.app.metadata[0].name
  }
  type = "kubernetes.io/dockerconfigjson"
  data = {
    ".dockerconfigjson" = jsonencode({
      auths = {
        "https://ghcr.io" = {
          username = var.image_pull_username
          password = var.image_pull_secret
          auth     = base64encode("${var.image_pull_username}:${var.image_pull_secret}")
        }
      }
    })
  }
}

resource "kubernetes_secret" "app" {
  metadata {
    name      = "app-secret"
    namespace = kubernetes_namespace.app.metadata[0].name
  }
  type = "Opaque"
  data = {
    "DB_URL"              = "postgres://${random_string.sql_admin_username.result}:${random_password.sql_admin_password.result}@${var.pgql_database}-postgresql.${kubernetes_namespace.pgql.metadata[0].name}.svc:5432/${var.pgql_database}"
    "DB_LOGGING"          = "true"
    "CORS_ORIGIN"         = "https://dealing-with-devops.westeurope.cloudapp.azure.com"
    "RUST_LOG"            = "info"
    "AUTH_PEPPER"         = "super-geheimer-pepper"
    "AUTH_ISSUER"         = "https://dealing-with-devops.westeurope.cloudapp.azure.com"
    "AUTH_JWT_EXPIRATION" = "604800"
    "NATS_URL"            = "nats://nats.${kubernetes_namespace.nats_ns.metadata[0].name}.svc:4222"
    "NATS_UPDATE_SUBJECT" = "game_room"
  }
}

resource "kubernetes_deployment" "backend" {
  metadata {
    name      = "backend"
    namespace = kubernetes_namespace.app.metadata[0].name
  }
  spec {
    replicas = 1
    selector {
      match_labels = {
        app = "backend"
      }
    }
    strategy {
      type = "RollingUpdate"
      rolling_update {
        max_surge       = 1
        max_unavailable = 0
      }
    }
    template {
      metadata {
        labels = {
          app = "backend"
        }
      }
      spec {
        image_pull_secrets {
          name = kubernetes_secret.image_pull.metadata[0].name
        }
        container {
          name              = "backend"
          image             = "ghcr.io/whiteduck-classroom/dealing-with-devops/backend:${var.image_tag}"
          image_pull_policy = "Always"
          tty               = true
          env_from {
            secret_ref {
              name = kubernetes_secret.app.metadata[0].name
            }
          }
          port {
            container_port = 8000
          }
        }
      }
    }
  }
}

resource "kubernetes_deployment" "frontend" {
  metadata {
    name      = "frontend"
    namespace = kubernetes_namespace.app.metadata[0].name
  }
  spec {
    replicas = 1
    selector {
      match_labels = {
        app = "frontend"
      }
    }
    strategy {
      type = "RollingUpdate"
      rolling_update {
        max_surge       = 1
        max_unavailable = 0
      }
    }
    template {
      metadata {
        labels = {
          app = "frontend"
        }
      }
      spec {
        image_pull_secrets {
          name = kubernetes_secret.image_pull.metadata[0].name
        }
        container {
          name              = "frontend"
          image             = "ghcr.io/whiteduck-classroom/dealing-with-devops/frontend:${var.image_tag}"
          image_pull_policy = "Always"
          port {
            container_port = 3000
          }
        }
      }
    }
  }
}

resource "kubernetes_service" "backend" {
  metadata {
    name      = "backend"
    namespace = kubernetes_namespace.app.metadata[0].name
  }
  spec {
    selector = {
      app = "backend"
    }
    port {
      port        = 8000
      target_port = 8000
    }
    type = "ClusterIP"
  }
}

resource "kubernetes_service" "frontend" {
  metadata {
    name      = "frontend"
    namespace = kubernetes_namespace.app.metadata[0].name
  }
  spec {
    selector = {
      app = "frontend"
    }
    port {
      port        = 3000
      target_port = 3000
    }
    type = "ClusterIP"
  }
}

resource "kubernetes_ingress_v1" "backend" {
  metadata {
    name      = "backend"
    namespace = kubernetes_namespace.app.metadata[0].name
    annotations = {
      "cert-manager.io/cluster-issuer" = var.cert_issuer_prod
      "nginx.ingress.kubernetes.io/rewrite-target" = "/$1"
    }
  }
  spec {
    ingress_class_name = var.ingress_class
    rule {
      host = "dealing-with-devops.westeurope.cloudapp.azure.com"
      http {
        path {
          path      = "/backend/(.*)"
          path_type = "Prefix"
          backend {
            service {
              name = kubernetes_service.backend.metadata[0].name
              port {
                number = 8000
              }
            }
          }
        }
      }
    }
    tls {
      hosts       = ["dealing-with-devops.westeurope.cloudapp.azure.com"]
      secret_name = "app-tls"
    }
  }
}

resource "kubernetes_ingress_v1" "frontend" {
  metadata {
    name      = "frontend"
    namespace = kubernetes_namespace.app.metadata[0].name
    annotations = {
      "cert-manager.io/cluster-issuer" = var.cert_issuer_prod
    }
  }
  spec {
    ingress_class_name = var.ingress_class
    rule {
      host = "dealing-with-devops.westeurope.cloudapp.azure.com"
      http {
        path {
          path      = "/"
          path_type = "Prefix"
          backend {
            service {
              name = kubernetes_service.frontend.metadata[0].name
              port {
                number = 3000
              }
            }
          }
        }
      }
    }
    tls {
      hosts       = ["dealing-with-devops.westeurope.cloudapp.azure.com"]
      secret_name = "app-tls"
    }
  }
}
