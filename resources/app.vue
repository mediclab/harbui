<template>
  <NuxtLayout>
    <header>
      <Header/>
    </header>
    <main id="main" class="mb-auto pb-8">
      <NuxtPage/>
    </main>
    <footer
        class="fixed bottom-0 left-0 z-20 w-full p-4 bg-white border-t border-gray-200 shadow md:flex md:items-center md:justify-between md:p-6">
      <Footer :version="version"/>
    </footer>
  </NuxtLayout>
</template>

<script setup>
import {ref} from 'vue'

const version = ref('##HARBUI_VERSION##')

</script>
