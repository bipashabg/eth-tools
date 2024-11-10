<template>
  <v-app dark>
    <v-navigation-drawer app color="#191825">
      <v-list-item>
        <v-list-item-content>
          <v-list-item-title class="text-h5">ETH Tools</v-list-item-title>
          <v-list-item-subtitle>Select a tool</v-list-item-subtitle>
        </v-list-item-content>
      </v-list-item>

      <v-divider></v-divider>

      <v-list dense>
        <v-list-item
          v-for="tool in tools"
          :key="tool.value"
          @click="selectedTool = tool.value"
          :active="selectedTool === tool.value"
          class="menu-item"
        >
          <v-list-item-title>{{ tool.title }}</v-list-item-title>
        </v-list-item>
      </v-list>
    </v-navigation-drawer>

    <v-app-bar app color="#191825">
      <v-toolbar-title>{{ currentToolTitle }}</v-toolbar-title>
      <v-spacer></v-spacer>
    </v-app-bar>

    <v-main>
      <v-container class="my-5">
        <component :is="currentToolComponent" />
      </v-container>
    </v-main>
  </v-app>
</template>

<script>
import AbiConverter from "@/components/AbiConverter.vue";
import HexDecimalConverter from "@/components/HexDecimalConverter.vue";
import ToolConverter from "@/components/ToolConverter.vue";
import UnitConverter from "@/components/UnitConverter.vue";
import BlockConverter from "@/components/BlockConverter.vue";
import StringConverter from "@/components/StringConverter.vue";
import AbiDecoder from "@/components/AbiDecoder.vue";
export default {
  name: "App",
  components: {
    AbiConverter,
    HexDecimalConverter,
    ToolConverter,
    UnitConverter,
    BlockConverter,
    StringConverter,
    AbiDecoder
  },
  data() {
    return {
      selectedTool: "abi",
      tools: [
        { title: "ABI Encoder", value: "abi" },
        { title: "ABI Decoder", value: "abi-dec" },
        { title: "Type Conversions", value: "type" },
        { title: "Transaction Decoder", value: "hash" },
        { title: "Unit Conversions", value: "unit" },
        { title: "Block Number to Timestamp", value: "block" },
        { title: "String Upper/Lower Case", value: "string" }
      ]
    };
  },
  computed: {
    currentToolTitle() {
      const selectedToolData = this.tools.find((tool) => tool.value === this.selectedTool);
      return selectedToolData ? selectedToolData.title : "";
    },
    currentToolComponent() {
      switch (this.selectedTool) {
        case "abi":
          return "AbiConverter";
        case "abi-dec":
          return "AbiDecoder";
        case "type":
          return "HexDecimalConverter"; // Replace with Type Conversion logic in the future
        case "unit":
          return "UnitConverter";
        case "block":
          return "BlockConverter";
        case "string":
          return "StringConverter";
        default:
          return "ToolConverter";
      }
    }
  }
};
</script>

<style>
.v-application {
  background-color: #191825;
  font-family: 'Dongle', sans-serif;
  color: #EEEEEE;
  font-weight: bold;
}
.menu-item.v-list-item--active {
  background-color: #865DFF;
  font-weight: bold;
  font-family: 'Dongle', sans-serif;
}
</style>
