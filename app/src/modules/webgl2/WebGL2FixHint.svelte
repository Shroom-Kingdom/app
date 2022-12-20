<script lang="ts">
  import { detect } from 'detect-browser';

  const browser = detect();
</script>

{#if !browser}
  <div>
    Please visit{' '}
    <a
      href="https://get.webgl.org/webgl2/"
      target="_blank"
      rel="noreferrer noopener"
    >
      this website
    </a>{' '}
    to get instructions how to enable WebGL2.
  </div>
{:else if browser.os === 'iOS'}
  <div>
    You can enable WebGL2 for Safari under &apos;Settings&apos; &gt;
    &apos;Experimental Features&apos; &gt; &apos;WebGL 2.0&apos;
  </div>
{:else}
  <div>
    Please visit{' '}
    <a
      href="https://get.webgl.org/webgl2/"
      target="_blank"
      rel="noreferrer noopener"
    >
      this website
    </a>{' '}
    to get instructions how to enable WebGL2.
  </div>
{/if}
