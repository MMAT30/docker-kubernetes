# BUILD
# docker build -t kub-first-app .
# docker tag kub-first-app mmat30/kub-first-app
# docker push mmat30/kub-first-app

# DEPLOYMENT
# kubectl create deployment first-app --image=mmat30/kub-first-app
# kubectl get deployments
# kubectl get pods

# EXPOSING
# kubectl expose deployment first-app --type=LoadBalancer --port=8080
# kubectl get services
# minikube service first-app

# SCALING
# kubectl scale deployment/first-app --replicas=3

# UPDATING
# kubectl set image deployment/first-app kub-first-app=mmat30/kub-first-app:2
# kubectl rollout status deployment/first-app

# ROLLBACK
# kubectl rollout history deployment/first-app --revision=2
# kubectl rollout undo deployment/first-app --to-revision=latest

FROM node:14-alpine

WORKDIR /app

COPY package.json .

RUN npm install

COPY . .

EXPOSE 8080

CMD [ "node", "app.js" ]