export function initMonaco(elementId, content, language , startLine = 1 , endline = 0) {
  require.config({ paths: { 'vs': 'https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.41.0/min/vs' } });
  require(['vs/editor/editor.main'], function () {
    const container = document.getElementById(elementId);

    const editor = monaco.editor.create(container, {
      value: content,
      language,
      automaticLayout: true,
      minimap: { enabled: false }, // Disable minimap for better height control
      scrollbar: { vertical: "hidden", horizontal: "auto" },
      theme: "hc-black",
      readOnly: true, // Makes the editor read-only
      lineNumbers: (line) => line + 0, //
    //   lineNumbers: 2,
    });


    console.log("haha",startLine, endline);

    let decorations =  [{
        range: new monaco.Range(startLine, 1, endline || startLine , 1),
        options: { isWholeLine: true, className: 'bg-sky-950' }
    }];

    function updateEditorHeight() {
      const lineCount = editor.getModel().getLineCount();
      const lineHeight = editor.getOption(monaco.editor.EditorOption.lineHeight);
      const padding = 5; // Extra padding for better spacing
      const newHeight = lineCount * lineHeight + padding;
      
      container.style.height = `${newHeight}px`;
      editor.layout();
    }

    editor.revealLineInCenter(startLine);
    editor.setPosition({ lineNumber: startLine , column: 1 });
    editor.deltaDecorations([], decorations);
    editor.getModel().onDidChangeContent(updateEditorHeight);
    updateEditorHeight(); // Initial adjustment
  });
}
