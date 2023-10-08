"use client";

import { SearchbarProvider } from "@/context/searchbar";

const providers = [SearchbarProvider];

/**
 * This component is used to wrap the entire application with all the context providers.
 */
export default function ContextProvider({ children }) {
  return providers.reduce((acc, Provider) => {
    return <Provider>{acc}</Provider>;
  }, children);
}
