import Link from 'next/link';
import { getPosts } from '../utils/mdx-utils';

import Footer from '../components/Footer';
import Header from '../components/Header';
import Layout, { GradientBackground } from '../components/Layout';
import ArrowIcon from '../components/ArrowIcon';
import { getGlobalData } from '../utils/global-data';
import SEO from '../components/SEO';


import React, { useReducer } from "react";
import DropZone from "../components/DropZone";
import styles from "../styles/Home.module.css";
import BitcoinAddressForm from "../components/TextBox";

export default function Index({ posts, globalData }) {
    // reducer function to handle state changes
    const reducer = (state, action) => {
      switch (action.type) {
        case "SET_IN_DROP_ZONE":
          return { ...state, inDropZone: action.inDropZone };
        case "ADD_FILE_TO_LIST":
          return { ...state, fileList: state.fileList.concat(action.files) };
        default:
          return state;
      }
    };
  
    // destructuring state and dispatch, initializing fileList to empty array
    const [data, dispatch] = useReducer(reducer, {
      inDropZone: false,
      fileList: [],
    });

  return (
    <Layout>
      <SEO title={globalData.name} description={globalData.blogTitle} />
      <Header name={globalData.name} />
      <main className="w-full">
        <h1 className={styles.title}>Create your own Bitcoin Ordinals</h1>
          {/* Pass state data and dispatch to the DropZone component */}
          <DropZone data={data} dispatch={dispatch} />
          <BitcoinAddressForm/>
      </main>
      <Footer copyrightText={globalData.footerText} />
      <GradientBackground
        variant="large"
        className="fixed top-20 opacity-40 dark:opacity-60"
      />
      <GradientBackground
        variant="small"
        className="absolute bottom-0 opacity-20 dark:opacity-10"
      />
    </Layout>
  );
}

export function getStaticProps() {
  const posts = getPosts();
  const globalData = getGlobalData();

  return { props: { posts, globalData } };
}
