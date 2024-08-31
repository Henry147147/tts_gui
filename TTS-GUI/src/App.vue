<script setup>
import Editor from './components/Editor.vue'
import FileSelector from './components/FileSelector.vue'
import SettingPage from './components/SettingPage.vue'
</script>

<template>
  <main>
    <div id="main-window">
      <FileSelector id="file-window" />
      <Editor />
      <SettingPage id="settings-window" />
    </div>
  </main>
</template>

<style scoped>
#main-window {
  display: flex;
  flex-direction: row;
  flex-wrap: nowrap;
  height: 100vh;
  width: 100vw;
  justify-content: space-between;
  align-items: flex-start;
}

#file-window {
  max-width: 25vw;
}

#settings-window {
  max-width: 25vw;
}
</style>
