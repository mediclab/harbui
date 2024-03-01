<template>
  <tr class="even:bg-gray-50" v-for="manifest in manifests">
    <td class="relative py-4 pr-3 text-sm font-medium text-gray-900 px-4 sm:px-6 lg:px-8">
      <div>{{ manifest.digest }}</div>
      <div class="absolute bottom-0 right-full h-px w-screen bg-gray-100"></div>
      <div class="absolute bottom-0 left-0 h-px w-screen bg-gray-100"></div>
    </td>
    <td class="hidden px-3 py-4 text-sm text-gray-500 sm:table-cell">{{ manifest.os }}/{{ manifest.architecture }}</td>
    <td class="hidden px-3 py-4 text-sm text-gray-500 sm:table-cell">{{ manifest.author }}</td>
    <td class="hidden px-3 py-4 text-sm text-gray-500 sm:table-cell">{{ humanFileSize(manifest.total_size) }}</td>
  </tr>
</template>

<script setup>
const props = defineProps({
  manifests: Array,
})

function humanFileSize(bytes, si = false, dp = 1) {
  const thresh = si ? 1000 : 1024;

  if (Math.abs(bytes) < thresh) {
    return bytes + ' B';
  }

  const units = si
      ? ['kB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB']
      : ['KiB', 'MiB', 'GiB', 'TiB', 'PiB', 'EiB', 'ZiB', 'YiB'];
  let u = -1;
  const r = 10 ** dp;

  do {
    bytes /= thresh;
    ++u;
  } while (Math.round(Math.abs(bytes) * r) / r >= thresh && u < units.length - 1);


  return bytes.toFixed(dp) + ' ' + units[u];
}
</script>
