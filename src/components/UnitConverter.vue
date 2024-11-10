<template>
  <v-card class="pa-5">
    <h3>ETH Unit Converter</h3>

    <v-text-field
      label="Wei (10⁻¹⁸)"
      id="wei"
      placeholder="0"
      v-model="abiTypes"
      outlined
    ></v-text-field>

    <v-text-field
      label="Gwei (10⁻⁹)"
      id="gwei"
      placeholder="0"
      v-model="abiTypes"
      outlined
    ></v-text-field>


    <v-text-field
      label="Ether"
      id="eth"
      placeholder="0"
      v-model="abiTypes"
      outlined
    ></v-text-field>

    <v-text-field
      label="USD"
      id="usd"
      placeholder="0"
      v-model="abiTypes"
      outlined
    ></v-text-field>

  </v-card>
</template>

<script>

</script>

<style scoped>
h3 {
  font-weight: bold;
  margin-bottom: 20px;
  font-family: 'Dongle', sans-serif;
}
</style>
