import { config } from "../config";
import axiosGlobal from "axios";

export const gatewayClient = axiosGlobal.create({
  baseURL: config.gatewayUrl,
  withCredentials: true
})
