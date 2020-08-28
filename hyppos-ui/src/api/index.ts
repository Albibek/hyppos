import { gatewayClient } from "./client";

function logout() {
  return gatewayClient.get("/auth/logout")
}


export const api = {
  logout
}
