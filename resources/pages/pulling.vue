<template>
  <div class="container w-full mx-auto pt-20">
    <div class="w-full px-4 md:px-0 md:mt-8 mb-16 text-gray-800 leading-normal">
      <div class="flex flex-row flex-wrap flex-grow mt-2">
        <div class="w-full p-3">
          <div class="bg-white border rounded shadow">
            <div class="border-b p-3">
              <h5 class="font-bold uppercase text-gray-600">Pulling from registry</h5>
            </div>
            <div class="flow-root overflow-hidden">
              <div class="mx-auto">
                <p class="px-4 py-2 text-gray-600 text-base">Viewing any Image from the Repositories menu will give you
                  a command in the following format:</p>
                <div class="my-2 mx-4 p-4 font-mono bg-gray-100">
                  docker pull {{ config?.registry_domain }}/&lt;someimage&gt;:&lt;tag&gt;
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
const {data: config} = await useLazyFetch(`/api/config`, {
  server: false,
})
</script>