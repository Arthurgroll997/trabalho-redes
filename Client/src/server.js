import { ApiAdapter } from "./adapter/api_adapter.js";
import express from "express";

const server = new ApiAdapter(process.env.PORT, express());

server.start();