<template>
  <div class="bg-white border rounded shadow p-2">
    <div class="flex flex-row items-center">
      <div class="flex-shrink pr-4">
        <div class="rounded p-3 bg-sky-600">
          <font-awesome-icon :icon="'fa-solid ' + icon" class="fa-2x fa-fw fa-inverse"/>
        </div>
      </div>
      <div class="flex-1 text-right md:text-center">
        <h5 class="font-bold uppercase text-gray-500">{{ title }}</h5>
        <h3 class="font-bold text-3xl">{{ metric }}</h3>
      </div>
    </div>
  </div>
</template>

<script setup>
const props = defineProps({
  title: String,
  metric: Number,
  icon: String,
})
</script>