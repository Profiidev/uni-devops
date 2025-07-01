resource "kubernetes_namespace" "pgql" {
  metadata {
    name = "pgql"
  }
}

resource "random_password" "sql_admin_password" {
  length  = 16
  special = false
}

resource "random_string" "sql_admin_username" {
  length  = 8
  lower   = true
  numeric = true
  special = false
}

resource "helm_release" "pgql" {
  name       = "pgql"
  repository = "oci://registry-1.docker.io/bitnamicharts"
  chart      = "postgresql"
  version    = "16.7.14"
  namespace  = kubernetes_namespace.pgql.metadata[0].name

  values = [templatefile("${path.module}/templates/pgql.values.tftpl", {
    database = var.pgql_database
    username = random_string.sql_admin_username.result
    password = random_password.sql_admin_password.result
  })]

  depends_on = [kubernetes_namespace.pgql]
}
