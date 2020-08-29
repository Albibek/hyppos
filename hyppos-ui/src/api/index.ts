import { gatewayClient } from "./client";
import { from } from "rxjs";

function logout() {
  return from(gatewayClient.get("/auth/logout"))
}


export const api = {
  logout
}
