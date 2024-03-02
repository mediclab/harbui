<template>
  <div class="container w-full mx-auto pt-20">
    <div class="w-full px-4 md:px-0 md:mt-8 mb-16 text-gray-800 leading-normal">
      <div class="flex flex-wrap">
        <div class="w-full md:w-1/2 xl:w-1/3 p-3">
          <MetricCard title="Total Images" :metric="pending_repos ? 0 : repos?.count ?? 0" icon="fa-hard-drive"/>
        </div>
        <div class="w-full md:w-1/2 xl:w-1/3 p-3">
          <MetricCard title="Total Users" :metric="pending_users ? 0 : users?.count ?? 0" icon="fa-users"/>
        </div>
      </div>

      <div class="flex flex-row flex-wrap flex-grow mt-2">
        <div class="w-full p-3">
          <div class="bg-white border rounded shadow">
            <div class="border-b p-3">
              <h5 class="font-bold uppercase text-gray-600">Images</h5>
            </div>
            <div class="flow-root overflow-hidden">
              <div class="mx-auto" v-if="pending_repo_tags">
                <div class="text-center p-4">
                  <Loader size="small"/>
                </div>
              </div>
              <div class="mx-auto" v-else>
                <div v-if="repo_tags.length === 0">
                  <EmptyTableState class="p-6"/>
                </div>
                <div v-else>
                  <table class="w-full text-left">
                    <thead class="bg-white">
                    <tr>
                      <th scope="col"
                          class="relative isolate py-3.5 pr-3 text-left text-sm font-semibold text-gray-900 px-4 sm:px-6 lg:px-8">
                        Name
                        <div class="absolute inset-y-0 right-full -z-10 w-screen border-b border-b-gray-200"></div>
                        <div class="absolute inset-y-0 left-0 -z-10 w-screen border-b border-b-gray-200"></div>
                      </th>
                      <th scope="col"
                          class="hidden px-3 py-3.5 text-left text-sm font-semibold text-gray-900 sm:table-cell">Tags
                      </th>
                    </tr>
                    </thead>
                    <tbody class="bg-white">
                    <tr class="even:bg-gray-50" v-for="repo in repo_tags">
                      <td class="relative py-4 pr-3 text-sm font-medium text-gray-900 px-4 sm:px-6 lg:px-8">
                        <NuxtLink :to="'/image/' + repo.image" class="hover:text-sky-600">{{
                            repo.image
                          }}
                        </NuxtLink>
                        <div class="absolute bottom-0 right-full h-px w-screen bg-gray-100"></div>
                        <div class="absolute bottom-0 left-0 h-px w-screen bg-gray-100"></div>
                      </td>
                      <td class="hidden px-3 py-4 text-sm text-gray-500 sm:table-cell">{{ repo.tags.join(', ') }}</td>
                    </tr>
                    </tbody>
                  </table>
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
import {notify} from "notiwind";

const {pending: pending_users, data: users} = await useLazyFetch('/api/count/users', {
  server: false
})

const {pending: pending_repos, data: repos} = await useLazyFetch('/api/count/repositories', {
  server: false
})

const {pending: pending_repo_tags, data: repo_tags} = await useLazyFetch('/api/repositories', {
  server: false,
})

onMounted(() => {
  notify(
      {
        group: "top",
        title: "Success",
        text: "Your account was created 👌",
      },
      -1
  );
})

</script>