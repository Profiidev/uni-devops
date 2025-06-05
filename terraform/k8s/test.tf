resource "kubectl_manifest" "test_deploy" {
  yaml_body = <<YAML
apiVersion: apps/v1
kind: Deployment
metadata:
  name: hello-world-deployment
  labels:
    app: hello-world
spec:
  selector:
    matchLabels:
      app: hello-world
  replicas: 1
  template:
    metadata:
      labels:
        app: hello-world
    spec:
      containers:
      - name: hello-world
        image: bhargavshah86/kube-test:v0.1
        ports:
        - containerPort: 80
        resources:
          limits:
            memory: 256Mi
            cpu: "250m"
          requests:
            memory: 128Mi
            cpu: "80m"
  YAML
}

resource "kubectl_manifest" "test_service" {
  yaml_body = <<YAML
apiVersion: v1
kind: Service
metadata:
  name: hello-world
spec:
  selector:
    app: hello-world
  ports:
    - protocol: TCP
      port: 80
      targetPort: 80
      nodePort: 30081   
  type: NodePort
  YAML
}

resource "kubectl_manifest" "test_ingress" {
  yaml_body = <<YAML
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: hello-world-ingress
  annotations:
    cert-manager.io/cluster-issuer: "${var.cert_issuer_prod}"
spec:
  ingressClassName: ${var.ingress_class}
  rules:
  - host: dealing-with-devops.westeurope.cloudapp.azure.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: hello-world
            port:
              number: 80
  tls:
  - hosts:
    - dealing-with-devops.westeurope.cloudapp.azure.com
    secretName: hello-world-tls
  YAML
}
