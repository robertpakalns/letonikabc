<script lang="ts">
    import { navigateToAbout } from "@/router";

    const { goBack } = $props<{
        goBack: () => void;
    }>();

    navigateToAbout();
</script>

<div class="centerWrapper">
    <div class="centered">
        <header>About LetonikaBC</header>

        <div class="text">
            <p>
                Source code:
                <a
                    href="https://github.com/robertpakalns/letonikabc"
                    target="_blank"
                >
                    https://github.com/robertpakalns/letonikabc
                </a>
            </p>
        </div>

        <button class="btn" onclick={goBack}>Back</button>
    </div>
</div>

<style>
    .text {
        display: flex;
        flex-direction: column;
        gap: 6px;
    }
</style>
