<script>
  import WebGL2FixHint from './WebGL2FixHint.svelte';
</script>

<div
style={{
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
  flexDirection: 'column',
  fontSize: '2rem',
  fontWeight: 500,
  textAlign: 'center'
}}
>
  <span>Your browser does not support WebGL2!</span>
  <WebGL2FixHint />
</div>