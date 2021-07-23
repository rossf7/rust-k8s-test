use isahc::{config::CaCertificate, prelude::*, Error, HttpClient};
use k8s_openapi::{List};
use k8s_openapi::api::core::v1::{Pod};
use serde_json;
use std::collections::BTreeMap;
use std::fs;
use urlencoding::encode;

#[derive(Debug)]
struct K8sContainer {
    container_id: String,
    container_name: String,
    namespace: String,
    labels: BTreeMap<String, String>,
    pod_name: String,
}

fn main() {
    let containers = generate_container_metadata();
    
    for c in containers {
        println!("{:?}", c)        
    }
}

fn generate_container_metadata() -> Result<Vec<K8sContainer>, Error> {
    let http_client = new_http_client().unwrap();

    let node_name = std::env::var("KUBERNETES_NODE_NAME").unwrap();
    let field_selector = format!("spec.nodeName={}", node_name);

    let service_host = std::env::var("KUBERNETES_SERVICE_HOST").unwrap();
    let service_port = std::env::var("KUBERNETES_SERVICE_PORT").unwrap();
    let request_url = format!("https://{}:{}/api/v1/pods?fieldSelector={}", service_host, service_port, encode(&field_selector));

    let response = http_client.get(request_url);
    let pods: List<Pod> = serde_json::from_str(&response.unwrap().text()?).unwrap();

    let mut containers: Vec<K8sContainer> = Vec::new();

    for p in pods.items {
        let labels = &p.metadata.labels;
        let namespace = &p.metadata.namespace.unwrap();
        let pod_name = &p.metadata.name.unwrap();

        for c in p.status.unwrap().container_statuses {
            let id: String = c.container_id.unwrap();
            containers.push(K8sContainer {
                container_id: id.split("/").last().unwrap().to_string(),
                container_name: c.name,
                labels: labels.clone(),
                namespace: namespace.to_string(),
                pod_name: pod_name.to_string()
            }) 
        }
    }

    Ok(containers)
}

fn new_http_client() -> Result<HttpClient, Error> {
	let service_account_token = fs::read_to_string("/var/run/secrets/kubernetes.io/serviceaccount/token")
	.expect("Fail to get service account token");
	return HttpClient::builder()
	.default_header("Authorization", format!("Bearer {}", service_account_token))
	.ssl_ca_certificate(CaCertificate::file("/var/run/secrets/kubernetes.io/serviceaccount/ca.crt"))
	.build();
}
