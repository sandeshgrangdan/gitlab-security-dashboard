export function initMonaco(elementId, content, language , startLine = 1) {
  require.config({ paths: { 'vs': 'https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.41.0/min/vs' } });
  require(['vs/editor/editor.main'], function () {
    const container = document.getElementById(elementId);

    const editor = monaco.editor.create(container, {
      value: content,
      language,
      automaticLayout: true,
      minimap: { enabled: true }, // Disable minimap for better height control
      scrollbar: { vertical: "hidden", horizontal: "auto" },
      theme: "vs-dark",
      readOnly: true, // Makes the editor read-only
      lineNumbers: (line) => line + startLine - 1, //
      scrollBeyondLastLine: false,
      mouseWheelScrollSensitivity: 0
    //   lineNumbers: 2,
    });


    function updateEditorHeight() {
      const lineCount = editor.getModel().getLineCount() + 0;
      const lineHeight = editor.getOption(monaco.editor.EditorOption.lineHeight);
      const padding = 0; // Extra padding for better spacing
      const newHeight = lineCount * lineHeight + padding;
      
      container.style.height = `${newHeight}px`;
      editor.layout();
    }

    editor.getModel().onDidChangeContent(updateEditorHeight);
    updateEditorHeight(); // Initial adjustment
  });
}
