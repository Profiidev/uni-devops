resource "azurerm_kubernetes_cluster" "k8s" {
  location            = var.location
  name                = var.name
  resource_group_name = var.resource_group_name
  dns_prefix          = var.name
  kubernetes_version  = "1.33"

  identity {
    type = "SystemAssigned"
  }

  default_node_pool {
    name       = "agentpool"
    vm_size    = "Standard_B2s"
    node_count = 1

    upgrade_settings {
      drain_timeout_in_minutes      = 0
      max_surge                     = "10%"
      node_soak_duration_in_minutes = 0
    }
  }

  network_profile {
    network_plugin    = "kubenet"
    load_balancer_sku = "basic"
  }
}

resource "azurerm_public_ip" "public_ip" {
  name                = var.name
  location            = var.location
  resource_group_name = azurerm_kubernetes_cluster.k8s.node_resource_group
  allocation_method   = "Static"
  sku                 = "Basic"
  domain_name_label   = var.name
}

resource "azurerm_role_assignment" "lb_ip_aks" {
  principal_id                     = azurerm_kubernetes_cluster.k8s.kubelet_identity[0].object_id
  role_definition_name             = "Network Contributor"
  scope                            = azurerm_kubernetes_cluster.k8s.node_resource_group_id
  skip_service_principal_aad_check = true
}
