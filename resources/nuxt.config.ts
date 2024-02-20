// @ts-ignore
export default defineNuxtConfig({
    devtools: {enabled: true},
    modules: ['@nuxtjs/tailwindcss'],
    buildModules: [
        '@nuxtjs/tailwindcss'
    ],
    css: [
        '@fortawesome/fontawesome-svg-core/styles.css'
    ],
    vite: {
        server: {
            proxy: {
                '/api': {
                    target: 'http://localhost:8000',
                    changeOrigin: true,
                }
            }
        }
    },
    experimental: {
        inlineSSRStyles: false
    },
    app: {
        buildAssetsDir: 'assets'
    },
    build: {
        transpile: [
            '@fortawesome/fontawesome-svg-core',
            '@fortawesome/free-solid-svg-icons',
            '@fortawesome/free-brands-svg-icons'
        ]
    },
    router: {
        options: {
            linkActiveClass: "text-sky-600 border-sky-500"
        }
    },
})
