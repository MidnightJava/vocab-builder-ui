import { ref } from 'vue'

export function useFetch(url) {
  const data = ref(null)
  const error = ref(null)

  fetch(url, {
    method: "GET",
    headers: {
        "Accept": "application/json"
    }
  })
    .then((res) => res.json())
    .then((json) => {
        data.value = json
    })
    .catch((err) => {
        error.value = err
    })

  return { data, error }
}