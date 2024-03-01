<template>
  <div class="container w-full mx-auto pt-20">
    <div class="w-full px-4 md:px-0 md:mt-8 mb-16 text-gray-800 leading-normal">
      <p class="p-3 font-bold text-sky-600 uppercase">{{ image }}</p>
      <div class="flex flex-row flex-wrap flex-grow mt-2" v-if="pending">
        <div class="w-full text-center p-4">
          <Loader size="big"/>
        </div>
      </div>
      <div class="flex flex-row flex-wrap flex-grow mt-2" v-else>
        <div class="w-full p-3" v-for="manifest in manifest_list">
          <div class="bg-white border rounded shadow">
            <div class="relative border-b p-3">
              <h5 class="font-bold uppercase text-gray-600">{{ manifest.tag }}</h5>
              <div class="absolute top-0 right-0 pt-1">
                <!--                <button-->
                <!--                    class="text-sm text-red-600 hover:text-white border border-red-600 hover:bg-red-600 font-medium rounded-lg px-3 py-2 text-center me-2 mb-2">-->
                <!--                  <font-awesome-icon icon="fa-solid fa-trash"/>-->
                <!--                </button>-->
              </div>
            </div>
            <div class="flow-root overflow-hidden">
              <TableManifests
                  :tag="manifest.tag"
                  :image="manifest.image"
                  :manifests="manifest.manifests ?? []"
                  :domain="config?.registry_domain"
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import {ref} from 'vue'

const route = useRoute()
const manifest_list = ref([]);
const pending = ref(true)
const image = `${route.params.user}/${route.params.name}`

const {data: tags} = await useLazyFetch(`/api/${image}/tags`, {
  server: false,
})

const {data: config} = await useLazyFetch(`/api/config`, {
  server: false,
})

onMounted(async () => {
  watch(tags, async () => {
    manifest_list.value = await Promise.all(tags.value.map((item) => $fetch(`/api/${image}/${item}`, {
      lazy: true,
      server: false,
    })));
    pending.value = false
  })
})
</script>