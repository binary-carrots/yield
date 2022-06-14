<script setup>
import HelloWorld from "./components/HelloWorld.vue";
// With the Tauri API npm package:
import { invoke } from "@tauri-apps/api/tauri";

const quitApp = () => {
  invoke("quit_app");
};

const sendMessage = () => {
  invoke("send_message");
};
</script>

<template>
  <HelloWorld msg="This is the start of yield" />
  <button @click="quitApp">quit app</button>
  <button @click="sendMessage">send message</button>
</template>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
