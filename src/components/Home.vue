<template>
    <v-app dark>
      <v-navigation-drawer app permanent color="primary" dark>
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
  
      <v-app-bar app color="primary" dark>
        <v-toolbar-title>{{ currentToolTitle }}</v-toolbar-title>
        <v-spacer></v-spacer>
      </v-app-bar>
  
      <v-main>
        <v-container class="my-5">
          <tool-converter
            :key="selectedTool"
            :title="currentToolTitle"
            :conversions="currentConversions"
          />
        </v-container>
      </v-main>
    </v-app>
  </template>
  
  <script>
  import ToolConverter from '@/components/ToolConverter.vue';
  
  export default {
    name: 'App',
    components: {
      ToolConverter
    },
    data() {
      return {
        selectedTool: 'abi',
        tools: [
          { title: 'ABI Encoder/Decoder', value: 'abi', conversions: [{ label: 'Encode', value: 'encode' }, { label: 'Decode', value: 'decode' }] },
          { title: 'Type Conversions', value: 'type', conversions: [{ label: 'Hex to Decimal', value: 'hexToDec' }, { label: 'Decimal to Hex', value: 'decToHex' }] },
          { title: 'Hash Decoder/Encoder', value: 'hash', conversions: [{ label: 'Hash Encode', value: 'hashEncode' }, { label: 'Hash Decode', value: 'hashDecode' }] },
          { title: 'Unit Conversions', value: 'unit', conversions: [{ label: 'Eth to Wei', value: 'ethToWei' }, { label: 'Wei to Eth', value: 'weiToEth' }] },
          { title: 'Block Number to Timestamp', value: 'block', conversions: [{ label: 'Block to Timestamp', value: 'blockToTimestamp' }] },
          { title: 'String Upper/Lower Case', value: 'string', conversions: [{ label: 'To Uppercase', value: 'toUpper' }, { label: 'To Lowercase', value: 'toLower' }] }
        ]
      };
    },
    computed: {
      currentToolTitle() {
        const selectedToolData = this.tools.find(tool => tool.value === this.selectedTool);
        return selectedToolData ? selectedToolData.title : '';
      },
      currentConversions() {
        const selectedToolData = this.tools.find(tool => tool.value === this.selectedTool);
        return selectedToolData ? selectedToolData.conversions : [];
      }
    }
  };
  </script>
  
  <style>
  .v-application {
    background-color: #121212;
  }
  .menu-item.v-list-item--active {
    background-color: rgba(255, 255, 255, 0.1);
  }
  </style>
  