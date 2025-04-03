<script lang="ts">
    import { Content } from "carbon-components-svelte";

    export let title: string;
</script>

<Content>
    <h1>{title}</h1>
    <p>Content for the {title} section will go here.</p>
</Content>
