<script lang="ts">
  import type Project from "src/models/project";
  import { invoke } from "@tauri-apps/api/tauri"

  const test = invoke("test").then(data => data as Project)
</script>

{#await test then value}
  {console.log(value)}
  
  <p>{value.name}</p>
  <img src={value.logo.thumbnailUrl} alt="" />
{/await}
