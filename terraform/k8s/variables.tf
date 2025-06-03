variable "k8s_host" {
  type = string
}

variable "k8s_certificate" {
  type = string
}

variable "k8s_key" {
  type = string
}

variable "k8s_cluster_ca_certificate" {
  type = string
}

variable "k8s_resource_group_name" {
  type = string
}

variable "public_ip_name" {
  type    = string
}

variable "ingress_class" {
  type    = string
  default = "nginx"
}
