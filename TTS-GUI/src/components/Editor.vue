<template>
  <ckeditor
    v-model="editorData"
    :editor="editor"
    :config="editorConfig"
    :style="{ height: '90vh;' }"
  />
</template>

<script>
import { ClassicEditor, Bold, Essentials, Italic, Mention, Paragraph, Undo } from 'ckeditor5'
import { Ckeditor } from '@ckeditor/ckeditor5-vue'

import 'ckeditor5/ckeditor5.css'

export default {
  name: 'app',
  components: {
    Ckeditor
  },
  data() {
    return {
      editor: ClassicEditor,
      editorData: '<p>Hello from CKEditor 5 in Vue!</p>',
      editorConfig: {
        plugins: [Bold, Essentials, Italic, Mention, Paragraph, Undo],
        toolbar: ['undo', 'redo', '|', 'bold', 'italic']
        // Other configuration options...
      }
    }
  }
}
</script>

<style lang="css">
.editor-container {
  height: 100vh; /* Make the container fill the viewport height */
}

.ck-editor__editable_inline {
  height: 100%; /* Make the editor content area fill its container */
}
</style>
