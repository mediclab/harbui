<template>
  <div class="mx-auto">
    <div class="text-left p-3 bg-gray-200">
      <span class="font-mono pl-2 overflow-hidden">docker pull {{ domain }}/{{ image }}:{{ tag }}</span>
    </div>
    <table class="w-full text-left">
      <thead class="bg-white">
      <tr>
        <th scope="col"
            class="relative isolate py-3.5 pr-3 text-left text-sm font-semibold text-gray-900 px-4 sm:px-6 lg:px-8">
          Image ID
          <div class="absolute inset-y-0 right-full -z-10 w-screen border-b border-b-gray-200"></div>
          <div class="absolute inset-y-0 left-0 -z-10 w-screen border-b border-b-gray-200"></div>
        </th>
        <th scope="col"
            class="hidden px-3 py-3.5 text-left text-sm font-semibold text-gray-900 sm:table-cell">
          Architecture
        </th>
        <th scope="col"
            class="hidden px-3 py-3.5 text-left text-sm font-semibold text-gray-900 sm:table-cell">Author
        </th>
        <th scope="col"
            class="hidden px-3 py-3.5 text-left text-sm font-semibold text-gray-900 sm:table-cell">Size
        </th>
      </tr>
      </thead>
      <tbody class="bg-white">
      <ItemManifest :manifests="manifests"/>
      </tbody>
    </table>
  </div>
</template>

<script setup>
const props = defineProps({
  image: String,
  tag: String,
  domain: String,
  manifests: Array,
})
</script>
