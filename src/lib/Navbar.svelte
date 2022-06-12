<script lang="ts">
</script>

<ul class="navbar">
  <li class="item selected">
    <button> All </button>
  </li>
  <li class="item">
    <button> Curseforge </button>
  </li>
  <li class="item">
    <button> FTB </button>
  </li>
  <li class="item">
    <button> ATLauncher </button>
  </li>
</ul>

<style>
  .navbar {
    display: flex;
    list-style: none;
    margin: 0;
    padding: 0;
  }
  
  .item {
    display: flex;
    flex-grow: 1;
    flex-basis: 0;
  }

  .item > * {
    flex-grow: 1;
    background: none;
    color: white;
    font-size: 32px;
  }

  .item:hover > * {
    background-color: hsl(0, 0%, 15%);
  }

  .item:active > * {
    color: rgb(150, 150, 150);
  }

  .selected {
    background-color: hsl(0, 0%, 10%);
  }
</style>
