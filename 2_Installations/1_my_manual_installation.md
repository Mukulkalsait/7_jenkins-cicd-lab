# 🟢 **1 – 100% Manual Setup with 3 Containers**

📌 Goal: **1 Master + 2 Agents – all Debians**
You will learn SSH connection + Jenkins agent configuration.

---

## 🧱 **Step 1 – Create 3 Containers (Podman)**

```bash
podman run -d --name jenkins-master --hostname jmaster -p 8080:8080 -p 50000:50000 docker.io/debian:latest sleep infinity
podman run -d --name agent1 --hostname jagent1 docker.io/debian:latest sleep infinity
podman run -d --name agent2 --hostname jagent2 docker.io/debian:latest sleep infinity
```

---

## 🛠 **Step 2 – Install Java + Jenkins in Master**

```bash
podman exec -it jenkins-master bash
apt update
apt install -y openjdk-11-jdk wget git ssh
wget -q -O /usr/share/keyrings/jenkins-keyring.asc \
  https://pkg.jenkins.io/debian/jenkins.io-2023.key

echo "deb [signed-by=/usr/share/keyrings/jenkins-keyring.asc] \
  https://pkg.jenkins.io/debian/ binary/" \
  > /etc/apt/sources.list.d/jenkins.list

apt update
apt install -y jenkins
systemctl enable jenkins
systemctl start jenkins
```

Check:

```bash
systemctl status jenkins
```

👉 Now open Jenkins at:
**[http://localhost:8080](http://localhost:8080)**

---

## 🔑 **Step 3 – Enable SSH-based Agent connection**

In `jenkins-master`:

```bash
ssh-keygen -t rsa
cat ~/.ssh/id_rsa.pub
```

Copy this key → we will paste into agents.

---

## ⚙️ **Step 4 – Configure Agent1 & Agent2**

```bash
podman exec -it agent1 bash
apt update && apt install -y openjdk-11-jdk ssh git
mkdir -p ~/.ssh
nano ~/.ssh/authorized_keys   # paste public key from master
chmod 600 ~/.ssh/authorized_keys
```

Repeat same steps for `agent2`.

Test connection:

```bash
ssh jagent1@agent1   # from master
ssh jagent2@agent2
```

---

## 🔗 **Step 5 – Add Agents in Jenkins UI**

1. Go to: `Manage Jenkins → Nodes`
2. New Node → `agent1`
3. Set:

   * `Remote root dir`: `/home/jenkins`
   * `Launch method`: *Launch agents via SSH*
   * Username: `root` (for testing)
   * Host: `agent1`
   * Credentials: add SSH key (paste your private key)

✔️ Once connected → you'll see **GREEN CONNECTED**
Now it's a **real Master-Agent setup**.

