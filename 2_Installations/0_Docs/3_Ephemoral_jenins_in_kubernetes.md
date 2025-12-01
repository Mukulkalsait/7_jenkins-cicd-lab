
# 🔵 ** 3 – REAL INDUSTRY METHOD**

### 🎯 **Ephemeral Jenkins Agents on Kubernetes**

Agents are created as **pods only when needed**, then deleted after job.

---

## 🚀 **Step 1 – Install Jenkins via HELM (Best Industry Way)**

```bash
helm repo add jenkins https://charts.jenkins.io
helm repo update
helm install jenkins jenkins/jenkins
```

Then:

```bash
kubectl get pods
kubectl port-forward svc/jenkins 8080:8080
```

Access: **[http://localhost:8080](http://localhost:8080)**

---

## ⚙️ **Step 2 – Install Kubernetes Plugin in Jenkins UI**

Go to:
`Manage Jenkins → Plugins → Kubernetes Plugin → Install`

---

## ⚡ **Step 3 – Add Kubernetes Cloud Configuration**

Go to:
`Manage Jenkins → Nodes & Clouds → Configure Clouds → Add Kubernetes`

Fill:

| Field          | Value                            |
| -------------- | -------------------------------- |
| Kubernetes URL | `https://kubernetes.default.svc` |
| Jenkins URL    | `http://jenkins:8080`            |
| Namespace      | default                          |
| Credentials    | Use ServiceAccountToken          |

---

## 🧪 **Step 4 – Create Ephemeral Agent Template**

In same page → **Pod Templates → Add Pod template**
Then fill:

```yaml
containers:
  - name: jnlp
    image: jenkins/inbound-agent
    args: ["${computer.jnlpmac}", "${computer.name}"]
```

✔️ Check **Run as ephemeral pod**
✔️ Label example: `k8s-agent`

---

## 🧪 **Step 5 – Write a Pipeline Using Ephemeral Agent**

```groovy
pipeline {
    agent { label 'k8s-agent' }

    stages {
        stage('Build') {
            steps {
                sh 'echo "Hello from Kubernetes agent!"'
            }
        }
    }
}
```

🔹 Every time you run this pipeline →
**A new pod is created**, job runs, pod auto-deletes.
That’s **true cloud CI/CD** 💯

---

# 🧠 **Which to Learn First?**

| Learning Order | Goal                                            |
| -------------- | ----------------------------------------------- |
| Option 1       | Learn Jenkins basics, SSH agents                |
| Option 2       | Fast setup using docker-compose                 |
| Option 3       | Industry-level CI/CD automation with Kubernetes |

---

Whenever you're ready — we can **start setting this up LIVE** ⬅️
Just say: **“Start Option 1 with me now”** and we begin step-by-step 🔥
