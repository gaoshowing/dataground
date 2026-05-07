<template>
  <main class="window-shell">
    <header class="title-bar">
      <div>
        <p class="eyebrow">Query Window</p>
        <h1>SQL Console</h1>
      </div>
      <span class="status-pill">Detached</span>
    </header>

    <section class="panel editor-preview">
      <h2>Query Editor</h2>
      <textarea spellcheck="false">select * from users limit 100;</textarea>
    </section>
  </main>
</template>
