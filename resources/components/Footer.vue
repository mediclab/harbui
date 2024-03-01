<template>
  <span class="text-sm text-gray-500 sm:text-center">v.{{
      version ?? 'dev'
    }} © 2024 MedicLab</span>
  <div class="mt-3 text-sm font-medium text-gray-500 sm:mt-0">
    <a href="https://github.com/mediclab" rel="nofollow" class="flex flex-wrap items-center">
      <font-awesome-icon icon="fa-brands fa-github" class="fa-2x pr-3"/>
      Follow me</a>
  </div>
</template>
<script setup>
const props = defineProps({
  version: String,
})
</script>